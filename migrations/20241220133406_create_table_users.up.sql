create table users (
    id bigserial primary key,
    name varchar(150) not null,
    phone varchar(14),
    role varchar(150),
    email varchar(190) not null,
    username varchar(30) not null,
    password bytea not null,
    created_at bigint not null default extract(
        epoch
        from now()
    ),
    updated_at bigint not null default extract(
        epoch
        from now()
    ),
    deleted_at bigint,
    unique(email),
    unique(username)
)