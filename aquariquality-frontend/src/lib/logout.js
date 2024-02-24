export const logout = async (sessionid) => {
    const json = JSON.stringify({"session_id": sessionid});
    fetch('http://127.0.0.1:8723/logout', {
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