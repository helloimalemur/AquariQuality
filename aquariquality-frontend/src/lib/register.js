// // curl -XPOST -H'X-API-KEY: omganotherone' localhost:8723/user/create/ -d '{"name":"johnny","email":"johhny@mail.com","password":"password"}'
export const register = async (name, email, password) => {
    const json = JSON.stringify({"name": name, "email": email, "password": password});
    fetch('http://127.0.0.1:8723/user/create/', {
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
