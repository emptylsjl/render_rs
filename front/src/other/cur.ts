import axios from "axios";

fetch('/cur').then(r => r.json()).then(data => {console.log(1, data)});
axios('/cur').then(function (response) {console.log(2, response)});

(async function() {
    let resp = await fetch('/cur');
    console.log(3, await resp.json());
})();

(async function() {
    const {data} = await axios('/cur');
    console.log(4, data);
})();

(async function() {
    let newXhr = new XMLHttpRequest();
    newXhr.open('post', '/cur');
    newXhr.onload = function (){console.log(5, newXhr.responseText)}
    newXhr.send()
})();

function xhrRq() {
    let newXhr = new XMLHttpRequest();
    newXhr.open("POST", "/main_api");
    newXhr.onload = function () {
        if (this.status === 200) {
            document.getElementById("abc").innerHTML = newXhr.responseText;
            console.log('good')
        }
    }
    newXhr.send()
}

async function rq(url, {method='GET', params, json, text, cache, credentials, xml, then}) {
    const strParams = Object.entries(params).map(([k, v]) => `${k}=${JSON.stringify(v).replace('&', '＆')}&`).join('').slice(0, -1)
    const reqUrl = url + (params? '?' + strParams : '');
    const reqBody = json? JSON.stringify(json) : undefined
    const header = (json||text)? {'Content-Type': `application/${json? 'json' : 'text'}`} : {}
    const reqOpt = {header, method, cache, credentials}.body = method === 'POST'? reqBody : undefined
    let resp = fetch(reqUrl, reqOpt)
    if (then) {then = resp.then}
    else {resp = await resp}
    if (!resp.ok) {return undefined}

    return {
        json: await resp.json(),
        text: await resp.text(),
        form: resp.formData(),
        code: resp.status,
        rdir: resp.redirect,
        then: then
    }
}



