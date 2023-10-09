export interface Issue{
    id: string,
    repository: string,
    issue: number,
    title: string,
    description?: string,
    createdAt: Date,
    closed: boolean
}