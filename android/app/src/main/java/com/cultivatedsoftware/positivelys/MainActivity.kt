package com.cultivatedsoftware.positivelys

import android.annotation.SuppressLint
import android.content.Intent
import android.os.Bundle
import android.util.Base64
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.appcompat.app.AppCompatActivity
import com.cultivatedsoftware.positivelys.models.AppContext
import com.cultivatedsoftware.positivelys.models.AppRequest
import com.cultivatedsoftware.positivelys.models.AppResponse
import com.cultivatedsoftware.positivelys.services.AppService
import com.google.gson.Gson

class MainActivity : AppCompatActivity() {



    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContentView(R.layout.activity_main)


        var databasePath = packageManager.getPackageInfo(packageName, 0).applicationInfo.dataDir
        val appRequest = AppRequest("https://positivelys.com/positivelys", "GET")

        val appService = AppService(databasePath)
        val response = appService.makeRequest(appRequest)
        val body = response.body ?: ""

        val myWebView: WebView = findViewById(R.id.webview)
        myWebView.addJavascriptInterface(WebAppInterface(this), "Android")
        myWebView.webViewClient = WebViewClient()
        myWebView.settings.javaScriptEnabled = true
        myWebView.loadDataWithBaseURL("https://positivelys.com", body, "text/html", "base64", "https://positivelys.com")
    }
}