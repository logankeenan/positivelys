package com.cultivatedsoftware.positivelys

import android.annotation.SuppressLint
import android.os.Bundle
import android.util.Base64
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.appcompat.app.AppCompatActivity
import com.cultivatedsoftware.positivelys.models.AppContext
import com.cultivatedsoftware.positivelys.models.AppRequest
import com.cultivatedsoftware.positivelys.models.AppResponse
import com.google.gson.Gson

class MainActivity : AppCompatActivity() {
    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        System.loadLibrary("rust_android")
        var databasePath = packageManager.getPackageInfo(packageName, 0).applicationInfo.dataDir


        val appRequest = AppRequest("https://positivelys.com/positivelys", "GET")
        val gson = Gson()
        val appRequestAsJson = gson.toJson(appRequest)
        val appContext = AppContext(database_path = "$databasePath/database.sqlite")
        val appContextAsJson = gson.toJson(appContext)

        val app_response_as_json = makeapprequest(appRequestAsJson, appContextAsJson)
        val response = gson.fromJson<AppResponse>(app_response_as_json, AppResponse::class.java)
        val body = response.body ?: ""

        val myWebView: WebView = findViewById(R.id.webview)
        myWebView.webViewClient = WebViewClient()
        myWebView.settings.javaScriptEnabled = true
        myWebView.loadData(Base64.encodeToString(body.toByteArray(), Base64.NO_PADDING), "text/html", "base64")

    }

    external fun makeapprequest(j_app_request: String, j_app_context: String): String
}