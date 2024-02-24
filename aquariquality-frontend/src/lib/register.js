export const register = async (email, password) => {
    const json = JSON.stringify({"email": email, "password": password});
    fetch('http://127.0.0.1:8723/register', {
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
        })
}
