CREATE TABLE changes(
    id integer primary key,
    timestamp datetime not null,
    author varchar(255) not null,
    changes_json text not null
)