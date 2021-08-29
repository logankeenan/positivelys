package com.cultivatedsoftware.positivelys

import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.TextView
import androidx.fragment.app.Fragment
import com.cultivatedsoftware.positivelys.models.AppRequest
import com.cultivatedsoftware.positivelys.services.AppService

const val WEB_PAGE_FRAGMENT_URL = "url"

class WebPageFragment(val fragmentUrl: String?) : Fragment(R.layout.fragment_web_page) {
    private lateinit var filesDirectory: String
    private lateinit var dataDirectory: String

    lateinit var webView: WebView

    override fun onPause() {
        super.onPause()

        Log.d("positivelys: ", "onPause fragmentUrl: $fragmentUrl")
    }

    override fun onResume() {
        super.onResume()

        Log.d("positivelys: ", "onResume fragmentUrl: $fragmentUrl")
    }

    fun setTitle() {

        Log.d("setTitle: ", "webView.title: ${webView.title} fragmentUrl: $fragmentUrl")
        val titleElement = this.activity?.findViewById<TextView>(R.id.toolbar_title)
        titleElement?.setText(webView.title.replace("Positivelys |", ""))
    }

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        // Inflate the layout for this fragment
        val inflate = inflater.inflate(R.layout.fragment_web_page, container, false)

        this.filesDirectory = inflate.context.filesDir.absolutePath
        this.dataDirectory = inflate.context.packageManager.getPackageInfo(
            inflate.context.packageName,
            0
        ).applicationInfo.dataDir

        WebView.setWebContentsDebuggingEnabled(true)
        webView = inflate.findViewById(R.id.fragment_web_view)
        webView.addJavascriptInterface(
            WebAppInterface(inflate.context as AppPageActivity),
            "Android"
        )
        val webViewClient = object : WebViewClient() {
            override fun onPageFinished(view: WebView?, url: String?) {
                super.onPageFinished(view, url)
                setTitle()
            }
        }

        webView.webViewClient = webViewClient
        webView.settings.javaScriptEnabled = true
        loadWebViewFromLocalServer()
        return inflate
    }

    private fun loadWebViewFromLocalServer() {

        // TODO historyUrl which is really failUrl should be a sorry somthing when wrong page
        val appRequest = AppRequest(fragmentUrl.toString(), "GET")
        val appService = AppService(this.dataDirectory, filesDirectory)
        val response = appService.makeRequest(appRequest)
        val body = response.body ?: ""
        webView.loadDataWithBaseURL(
            "file:///android_asset/",
            body,
            "text/html",
            "base64",
            "https://positivelys.com"
        )
    }

    fun reload() {
        Log.d("positivelys: ", "reload fragmentUrl: $fragmentUrl")
        loadWebViewFromLocalServer()
    }

}
