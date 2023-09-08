function postdata(){
    // フォームのデータを取得
    const formData = new FormData(document.getElementById("myForm"));

    // フォームデータが空でないかチェック
    if(count() === 0) {
        alert("お名前を入力してください。");
        return false;
    }
    else { 
        // actix-webのバックエンドにPOSTリクエストを送信
        fetch("/bbs", {
            method: "POST",
            body: formData
        })
        .then(data => {
            // レスポンスを処理するコードをここに追加
            console.log(data);
        })
        // 入力フォームをクリア
        document.getElementById("data").value = "";
    }
}

var textData = document.getElementById("data").value;

// フォーム送信時にpostdata関数を呼び出す
document.getElementById("myForm").addEventListener("submit", (event) => {
    event.preventDefault(); // フォームのデフォルトの送信動作を防ぎます
    postdata(); // postdata関数を呼び出します
    fetchData();//送信時にもデータを読み込む
});

//ページ読み込み時にもデータを取得
window.onload = function() {
    fetchData();
};

function count(){
    var input = document.getElementById('data');
    var count = input.value.replace(/\s+/g, "").length;
    return count;
}


// JavaScript
function fetchData() {
    fetch('/api/some_endpoint')
        .then(response => response.json())
        .then(data => {
        // データを表示するためのコードをここに追加する
        console.log(data);

        // "message"キーからメッセージ配列を取得
        const messages = data.message;

        // データを表示する要素のIDを持つHTML要素を取得
        const dataContainer = document.getElementById('data-container');

        // "data-container"をクリア
        dataContainer.innerHTML = "";

        // データを表示する要素のIDを持つHTML要素を取得
        const dataList = document.createElement("ul");

        dataContainer.appendChild(dataList);

        // データを要素に追加
        //dataElement.textContent = data.message; // ここでdata.messageはサーバーからのデータの一部と仮定
        for (let i = 0; i < messages.length; i++) {
            const postUl = document.createElement("ul");

            // メッセージ要素を生成
            const messageDiv = document.createElement("li");

            const messageText = document.createTextNode(messages[i]);

            //messageDiv.appendChild(strong);
            messageDiv.appendChild(messageText);

            // postDivにSVGとメッセージを追加
            dataList.appendChild(messageDiv);

        }
        })
        .catch(error => {
        console.error('Error:', error);
    });
}

// 一定の間隔でデータを取得
const interval = 5000; // 5秒ごとにデータを取得する例
setInterval(fetchData, interval);


