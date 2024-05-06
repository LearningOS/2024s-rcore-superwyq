## 实现细节
在syscall的mod.rs文件中添加syscall_mmap和syscall_munmap函数，这两个函数再调用task的mod.rs中的相应函数，这些函数放在task而不是mm的原因是因为需要获取当下的Runing的程序以方便找到该程序的MemorySet和page_table，在MemorySet下实现一个相应的功能函数mmap和munmap。
mamp的流程：
- 将根据start和len找到需要分配的虚拟页号VirtPageNum，对于start取floor，对于end取ceil
- 使用每个VirtAddr在page_table中查询是否有对应的pte，如果有表示已经分配过了，直接return -1，如果没有就使用frame_alloc函数进行物理页帧分配
- 使用page_table的map函数构建pte并放到page_table中

munmap流程：
- 将根据start和len找到需要释放的的虚拟页号VirtPageNum，对于start取floor，对于end取ceil
- 根据vpn查找是否存在对应的有效的pte，如果有，使用page_table的unmap函数释放掉，如果没有，return -1

需要考虑的特殊情况：
- start地址是否对齐
- prot是否有效
## 简答题


## 荣誉准则
在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
- None
此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
- https://rcore-os.cn/rCore-Tutorial-Book-v3/

我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。