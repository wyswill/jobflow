# mini CI tool

小巧的CI服务.

 在个人轻量级场景中替换jenkins 这种重型CI 工具.
 
  使用 100% safe Rust 实现. 实现内存占用小,cpu 占用小

沙箱实现: 在一个空的docker 容器中执行shell 脚本, 在容器中配置好需要的环境

[ ] 并发flow执行

[ ] 沙箱flow执行

[ ] flow 语法