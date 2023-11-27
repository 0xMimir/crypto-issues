update topics_repositories
set scraped = true
from (
        select tr.id
        from topics_repositories tr
            inner join github_projects gp on gp."name" = tr.repository_owner
            inner join github_repositories gr on gr.repository_name = tr.repository_name
            and gr.project = gp.id
        where not tr.scraped
    ) as subquery
where topics_repositories.id = subquery.id