select
  u.*,
  coalesce(array_agg((c.id, c.code)) filter (where c.id is not null), array[]::record[]) as "codes: Vec<Code>"
from
  users u
left join
  code c on u.id = c.user_id
group by
  u.id
