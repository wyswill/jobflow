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

