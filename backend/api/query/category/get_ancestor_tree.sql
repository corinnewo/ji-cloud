with recursive links as
                   (
                       select id,
                              parent_id
                       from category co
                       where id = any ($1::uuid[])
                       union all
                       select co.id,
                              co.parent_id
                       from category co
                                inner join links ct on (ct.parent_id = co.id)
                   )

select distinct id,
       category.parent_id,
       name,
       category.index,
       created_at,
       updated_at,
       (select count(*) from image_category where category_id = id)::int8 as "image_count!",
       (select count(distinct jig.id)::int8
        from jig
                 join jig_data_category on (jig.live_id = jig_data_id or jig.draft_id = jig_data_id)
        where category_id = id)                                           as "jig_count!",
        user_scopes
from category
         inner join links using (id);
