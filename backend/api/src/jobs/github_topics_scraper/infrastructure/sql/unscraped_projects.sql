select distinct tr.repository_owner as project_name
from topics_repositories tr
    full outer join github_projects gp on gp."name" = tr.repository_owner
where tr.scraped = false
    and gp.id isnull