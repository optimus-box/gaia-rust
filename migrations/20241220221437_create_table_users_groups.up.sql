create table users_groups (
    user_id bigint not null,
    group_id bigint not null,
    primary key (user_id, group_id),
    foreign key (user_id) references users (id) on delete cascade,
    foreign key (group_id) references groups (id) on delete cascade
)