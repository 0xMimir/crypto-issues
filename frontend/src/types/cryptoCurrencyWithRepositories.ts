export interface CryptoCurrencyWithRepositories{
    id: string,
    name: string,
    coingeckoId: string,
    githubId: string,
    github: string,
    repositories: Repository[],
    issues: number
}

export interface Repository{
    id: string,
    repositoryName: string,
    language?: string,
    stargazersCount: number,
    forksCount: number
}