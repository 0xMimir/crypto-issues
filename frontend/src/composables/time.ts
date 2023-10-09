export function formatDate(date: Date): string{
    const year = date.getFullYear();
    const month = date.getMonth();
    const day = date.getDate();
    return `${year}/${month}/${day}`;
}