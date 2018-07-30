-- This returns a random video + first, last, next, prev ids as a json object
create or replace function filter_random(filters text[], deleted boolean default false)
  returns setof json
  as
$$
with filtered as (
        select * from videos where not (tags && filters) and (deleted or deleted_at is null)
    ),
    raw as (
        select
            *,
            first_value(id) over ordered as first,
            last_value(id) over ordered as last,
            lag(id) over ordered as prev,
            lead(id) over ordered as next
        from
            filtered
        window
            ordered as (order by created_at asc rows between unbounded preceding and unbounded following)
        limit 1
        offset floor(random() * (select count(*) from filtered))
   )
   select
       row_to_json(t)
   from
       raw t
$$
language sql;
