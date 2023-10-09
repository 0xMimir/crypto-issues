export interface Pagination<T>{
    page: number,
    perPage: number,
    orderBy: string[],
    data: T[],
    totalItems: number,
    lastPage: number
}