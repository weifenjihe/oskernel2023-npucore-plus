/** @addtogroup NPUcore-IMPACT!!!
 * @{
 */
/**
 * @file  ext4_user.c
 * @brief Ext4 memory allocate without libc(LoongArch).
 */

#include <ext4_config.h>
#include <ext4_debug.h>
#include <ext4_errno.h>
#include <ext4_misc.h>
#include <ext4_types.h>

#include <ext4_block_group.h>
#include <ext4_crc32.h>
#include <ext4_super.h>
#include <ext4_trans.h>
#include <ext4_user.h>

// unsigned char buff[EXT4_USER_BUFF_SIZE];

t_block user_find_block(t_block *last, size_t size)
{
	t_block b = USER_HEAP_BASE; // 全局变量，堆起始地址
	while (b && !(b->free && b->size >= size)) {
		*last = b;
		b = b->next;
	}
	return b;
}

t_block user_extend_heap(t_block last, size_t size)
{
	t_block b;
	b = sys_sbrk(0); // 定位到当前break位置
	if (sys_sbrk(sizeof(struct s_block) + size) == (void *)-1)
		return NULL;
	b->size = size;
	b->next = NULL;
	if (last) // 这个last是指向extend之前最后一个block
		last->next = b; // 新开辟的block挂载在链表中
	b->free = 0;
	return b;
}

// b是要分裂的block，size是申请的内存大小
// 分裂后b成了分配后的block
void user_split_block(t_block b, size_t size)
{
	t_block new; // 新的空闲block = 要分裂的block - 申请分配出去的内存
	new = b->data + size; // 将new定位到剩下的数据块区域

	// 分裂的原block-分配出去的内存大小-block结构体本身大小
	new->size = b->size - size - EXT4_USER_BLOCK_SIZE;
	new->next = b->next; // 链表插入
	new->free = 1;	     // 空闲标记可用
	b->size = size;
	b->next = new; // 链表插入
}

// 合并相邻空闲的内存块，参数决定合并的是上一个还是下一个
t_block user_fusion(t_block b)
{
	if (b->next && b->next->free) {
		b->size += EXT4_USER_BLOCK_SIZE + b->next->size;
		b->next = b->next->next;
		if (b->next)
			b->next->prev = b;
	}
	return b;
}

// 注意，这个函数最后通过偏移量得到的block可能是有效的，可能不是有效的
t_block user_get_block(void *p)
{
	char *tmp;
	tmp = p;
	return (p = tmp -= EXT4_USER_BLOCK_SIZE);
}

size_t user_valid_addr(void *p)
{
	if (USER_HEAP_BASE) {
		if (p > USER_HEAP_BASE && p < sys_sbrk(0))
			return (p == (user_get_block(p))->ptr);
		// 如果两个字段地址一样，表示是一个有效block
	}
	return 0;
}

// void *ext4_user_malloc(size_t size)
// {
// 	t_block b, last;
// 	size_t s;

// 	s = ALIGN4_HI(size);
// 	if (USER_HEAP_BASE) {
// 		// first find a block
// 		last = USER_HEAP_BASE;
// 		b = user_find_block(&last, s);
// 		if (b) {
// 			// can we split
// 			if ((b->size - s) >= (EXT4_USER_BLOCK_SIZE + 8))
// 				user_split_block(b, s);
// 			b->free = 0;
// 		} else {
// 			// no fitting block, extend the heap
// 			b = user_extend_heap(last, s);
// 			if (!b)
// 				return NULL;
// 		}
// 	} else {
// 		// first time
// 		b = user_extend_heap(NULL, s);
// 		if (!b)
// 			return NULL;
// 		USER_HEAP_BASE = b;
// 	}
// 	return b->data;
// }

void *ext4_user_calloc(size_t numitems, size_t size)
{
	size_t *new;
	size_t s, i;
	new = ext4_user_malloc(numitems * size);
	if (new) {
		// 因为申请的内存总是4的倍数，所以这里我们以4字节为单位初始化
		s = ALIGN4_HI(numitems * size) >> 2;
		for (i = 0; i < s; ++i)
			new[i] = 0;
	}
	return new;
}

void ext4_user_free(void *p)
{
	t_block b;
	if (user_valid_addr(p)) // 地址的有效性验证
	{
		b = user_get_block(p); // 得到对应的block
		b->free = 1;

		// 如果相邻的上一块内存是空闲的就合并,
		// 合并之后的上一块还是空闲的就继续合并，直到不能合并为止
		while (b->prev && b->prev->free) {
			b = user_fusion(b->prev);
		}

		// 同理去合并后面的空闲block
		while (b->next)
			user_fusion(b); // 内部会判断是否空闲

		// 如果当前block是最后面的那个block，此时可以调整break指针了
		if (NULL == b->next) {
			if (b->prev) // 当前block前面还有占用的block
				b->prev->next = NULL;
			else // 当前block就是整个heap仅存的
				USER_HEAP_BASE = NULL; // 则重置base
			sys_brk(b); // 调整break指针到b地址位置
		}
		// 否则不能调整break
	}
}

// void *ext4_user_realloc(void *p, size_t size)
// {
// 	size_t s;
// 	t_block b, new;
// 	void *newp;

// 	if (!p)
// 		return ext4_user_malloc(size);
// 	if (user_valid_addr(p)) {
// 		s = ALIGN4_HI(size);
// 		b = get_block(p); // 得到对应的block
// 		if (b->size >= s) // 如果size变小了，考虑split
// 		{
// 			if (b->size - s >= (EXT4_USER_BLOCK_SIZE + 4))
// 			user_split_block(b, s);
// 		} else // 如果当前block的数据区不能满足size
// 		{
// 			// 如果后继block是free的，并且合并后大小满足size，考虑合并
// 			if (b->next && b->next->free &&
// 			    (b->size + EXT4_USER_BLOCK_SIZE + b->next->size) >=
// 				s) {
// 				user_fusion(b);
// 				// 合并后满足size，再看能不能split
// 				if (b->size - s >= (EXT4_USER_BLOCK_SIZE + 4))
// 					user_split_block(b, s);
// 			} else // 以上都不满足，则malloc新区域
// 			{
// 				newp = ext4_user_malloc(s);
// 				if (!newp)
// 					return NULL;
// 				// 内存复制
// 				new = user_get_block(newp);
// 				user_copy_block(b, new);
// 				ext4_user_free(p); // 释放old
// 				return newp;
// 			}
// 		}
// 		return p; // 当前block数据区大于size时
// 	}
// 	return NULL;
// }

size_t strlen(char *str)
{
	char *start = str;
	while (*str != '\0') {
		str++;
	}
	return str - start;
}

size_t strcpy(char *s1, char *s2)
{
	size_t len = strlen(s2);
	for (size_t i = 0; i < len; ++i) {
		s1[i] = s2[i];
	}
	s1[len] = 0;
}

size_t strcmp(char *s1, char *s2)
{
	while (*s1 == *s2 && *s1 != '\0') {
		s1++;
		s2++;
	}
	return *s1 - *s2;
}

size_t strncmp(char *str1, char *str2, size_t n)
{
	while (n) {
		if (*str1 != *str2)
			return *str1 - *str2;
		if (*str1 == '\0')
			return 0;
		str1++;
		str2++;
		n--;
	}
	return 0;
}

void swap(char *buf1, char *buf2, size_t size)
{
	size_t i = 0;
	char temp = 0;
	for (i = 0; i < size; i++) {
		temp = *buf1;
		*buf1 = *buf2;
		*buf2 = temp;
		buf1++;
		buf2++;
	}
}

// 先用bubble sort验证是否通过编译
void qsort(void *base, size_t num, size_t size,
	   size_t (*cmp)(const void *, const void *))
{
	size_t i = 0;
	for (i = 0; i < num - 1; i++) {
		size_t j = 0;
		for (j = 0; j < num - 1 - i; j++) {
			// 假设需要升序，>0就交换
			if (cmp((char *)base + j * size,
				(char *)base + (j + 1) * size) >
			    0) // 两个元素比较，需要将arr[j]，arr[j+1]的地址传给cmp
			{
				// 交换函数
				swap((char *)base + j * size,
				     (char *)base + (j + 1) * size, size);
			}
		}
	}
}

/**
 * @}
 */
