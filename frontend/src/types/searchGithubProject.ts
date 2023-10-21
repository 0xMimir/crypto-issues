export interface SearchGithubProject {
    id: string;
    name: string;
    repositories: number;
    languagesUsed: string[];
    issues: number;
    stargazersCount: number;
}

export interface SearchGithubProjectParams {
    page?: number;
    perPage?: number;
    order?: string;
    orderBy?: string;
    languagesUsed?: string[];
}