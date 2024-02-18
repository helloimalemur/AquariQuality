export const login = async (email, password) => {
    const json = JSON.stringify({"email": email, "password": password});
    let key;
    key = fetch('http://127.0.0.1:8723/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'X-API-KEY': 'omganotherone',
        },
        body: json,
    })
        .then((response) => response.text())
        .then((data) => {
            console.log(data)
            key = data;
            return key
        })
    return key
}
