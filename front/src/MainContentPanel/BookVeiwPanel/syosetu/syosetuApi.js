import axios from "axios";


// eslint-disable-next-line no-unused-vars
const syosetuDoc = {
    'gzip': ['int', 'int(1～5) for gzip level'],
    'out': ['str', 'specify yaml/json/php'],
    'of': ['str', ],
    'lim': ['int', 'int(1~500) for output amount'],
    'st': ['int', '(1～2000) for where start'],
    'order': ['str', 'specify output order (new/favnovelcnt/reviewcnt/hyoka/hyokaasc/dailypoint/weeklypoint/monthlypoint/quarterpoint/yearlypoint/impressioncnt/hyokacnt/hyokacntasc/weekly/lengthdesc/lengthasc/ncodedesc/old)'],
    'libtype': ['int', 'yaml only'],
    'updatetype': ['int', 'atom only'],
    'word': ['str', 'to include'],
    'notword': ['str', 'to exclude'],
    'title': ['int', '1 for word else notword'],
    'ex': ['int', '1 for word else notword'],
    'keyword': ['int', '1 for word else notword'],
    'wname': ['int', '1 for word else notword'],
    'biggenre': ['int/str', ],
    'notbiggenre': ['int/str', ],
    'genre': ['int/str', ],
    'notgenre': ['int/str', ],
    'userid': ['int/str', ],
    'isr15': ['int', ],
    'isbl': ['int', ],
    'isgl': ['int', ],
    'iszankoku': ['int', ],
    'istensei': ['int', ],
    'istenni': ['int', ],
    'istt': ['int', ],
    'notr15': ['int', ],
    'notbl': ['int', ],
    'notgl': ['int', ],
    'notzankoku': ['int', ],
    'nottensei': ['int', ],
    'nottenni': ['int', ],
    'minlen': ['int', ],
    'maxlen': ['int', ],
    'length': ['int/str', ],
    'kaiwaritu': ['int/str', ],
    'sasie': ['int/str', ],
    'mintime': ['int', ],
    'maxtime': ['int', ],
    'time': ['int/str', ],
    'ncode': ['str', ],
    'type': ['str', ],
    'buntai': ['int/str', ],
    'stop': ['int', ],
    'lastup': ['str', ],
    'ispickup': ['int', ],
}

// eslint-disable-next-line no-unused-vars
function strParam(param, jsonDump=true) {
    return (
        '?'+Object.entries(param).map(
            ([k, v]) => v?`${k}=${(jsonDump? JSON.stringify(v) : v).replace('&', '＆')}&`:''
        ).join('').slice(0, -1)
    )
}

async function SyosetuRq(
    {
        out='json', lim='50', st, order, word, notword,
        title, ex, keyword, wname, biggenre, notbiggenre, genre, notgenre,
        length, sasie, ncode
    }) {

    const url_param = {
        out: 'json',
        lim: '50',
        st, order, word: word, notword, title, ex, keyword, wname,
        biggenre, notbiggenre, genre, notgenre,
        length, sasie, ncode
    }

    // let rqUrl = syosetu_url + strParam(url_param, false);

    // return  fetch(rqUrl, {
    //     method: 'GET',
    // }).then(r => r.json()).then(d => {return d})

    console.log(3, url_param)

    const {data} = await axios("https://larrybookstuff.herokuapp.com/", {params: url_param});
    return data
}

export default SyosetuRq