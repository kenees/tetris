### 逻辑

1. main.rs 创建程序窗口
2. tetris_window 中关联 game, 绘制基础窗口， 调用 game 中的 update 更新游戏数据， 监听键盘事件触发 game 的 update
3. tetris_game 中保存游戏数据， 暴露暂停或继续接口， 暴露重新开始， 关联 tetris（多个方块）
4. tetris_tetris 中暴露单个方块的移动事件， 旋转事件， 位置信息等等， 添加碰撞检测

###

基本思路， 将场景分割为 10 x 20 个小方块（10x20）

1. 利用矩阵， 初始为 20 x 10 的矩阵，值为 0 表示未高亮， 1 为高亮

2. 随机创建一组包含一种形状的点数组， 默认第一个为旋转中心

3. 系统会不停的刷新画面， 刷新前会合并当前在活动的方块，与已经确定的方块。

4. 所有操作都是操作当前活动的方块， 移动，旋转都是改变相关坐标。（左上角为 (x:0 ,y: 0), 竖直方向为 x, 水平方向为 y）

5. 当判断到无法下降的时候表示当前活动方块生命周期结束了， 将其合并到已确定方块矩阵中，并重新创建一个活动方块，重复流程。

<!-- TODO -->

1. 美化 UI
2. 添加快速下降功能，暂时没有
3. 计分可以添加多行同时消除积分更高等等。
