create table groups (
    id bigserial primary key,
    name varchar(50) not null,
    description varchar(255),
    roles jsonb not null,
    created_at bigint not null default extract(
        epoch
        from now()
    ),
    updated_at bigint not null default extract(
        epoch
        from now()
    ),
    deleted_at bigint,
    unique(name)
)