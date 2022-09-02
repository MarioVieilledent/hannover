
// eslint-disable-next-line @typescript-eslint/ban-types
export function httpService(url: string, func: Function): void {
    console.warn(url);
    fetch(url, {
        method: 'GET',
        mode: 'cors'
    }).then(res => {
        res.json().then(data => {
            func(data);
        });
    })
}