## 实现细节
模仿其他的syscall实现方式，在task module中分别实现了add_syscall_times函数、get_syscall_times函数、task_start_time函数，并在TaskManger实现了相应的方法。
在TCB中添加了start_time属性和syscall_times属性
整体的工作流程为：每次syscall前都会先调用add_syscall_times函数，这个函数又调用TaskManager的add_syscall_times方法，该方法会修改当下Task的TCB中的syscall_times属性。当调用sys_task_info时，该系统调用会修改传入的TaskInfo指针。

## 简答题


## 荣誉准则
在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
- None
此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
- https://rcore-os.cn/rCore-Tutorial-Book-v3/

我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。