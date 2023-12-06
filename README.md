# mini CI tool

小巧的CI服务.

 在个人轻量级场景中替换jenkins 这种重型CI 工具.
 
  使用 100% safe Rust 实现. 实现内存占用小,cpu 占用小

沙箱实现: 在一个空的docker 容器中执行shell 脚本, 在容器中配置好需要的环境

启动命令: jobFlow [configPath]
 
 config 字段说明
 ```yml
# 指定mysql链接地址
db_url: mysql://[user]:[password]@[host]:[port]/[dbName]?connection_limit=10&pool_timeout=60
# 服务线程数量
server_worker_size: 2
# 服务端口
server_port: 8080
 ```

 建表sql
 ```sql
 create table flow
(
	id int auto_increment
		primary key,
	name varchar(40) null,
	create_time varchar(30) null,
	update_time varchar(30) null,
	shell_str longtext null
);

create table project
(
	id int auto_increment
		primary key,
	name varchar(30) null,
	create_time varchar(30) null,
	update_time varchar(30) null
);

create table project_flow
(
	id int auto_increment
		primary key,
	project_id int not null,
	flow_id int not null
);

 ```
