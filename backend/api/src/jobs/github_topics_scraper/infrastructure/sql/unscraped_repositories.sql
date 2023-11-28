select tr.repository_owner as project_name,
    tr.repository_name as repository_name,
    gp.id as repository_owner
from topics_repositories tr
    inner join github_projects gp on gp."name" = tr.repository_owner
where tr.scraped = false