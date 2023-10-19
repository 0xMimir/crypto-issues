export interface SearchRepository {
    id: string;
    repositoryName: string;
    language?: string;
    stargazersCount: number;
    forksCount: number;
    createdAt: Date;
    updatedAt: Date;
    archived: boolean;
    projectId: string;
    project: string;
    url: string;
}

export interface SearchRepositoryParams {
    orderBy?: string;
    order?: string;
    page?: number;
    perPage?: number;
    language?: string;
    repository?: string;
    projectId?: string;
    project?: string;
    archived?: boolean;
    fork?: boolean;
}