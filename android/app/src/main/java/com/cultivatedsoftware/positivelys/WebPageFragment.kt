package com.cultivatedsoftware.positivelys

import android.os.Bundle
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

class WebPageFragment : Fragment(R.layout.fragment_web_page) {
    var fragmentUrl: String? = null
    lateinit var webView: WebView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        arguments?.let {
            fragmentUrl = it.getString(WEB_PAGE_FRAGMENT_URL)
        }
   }

    fun setTitle() {
        val titleElement = this.activity?.findViewById<TextView>(R.id.toolbar_title)
        titleElement?.setText(webView.title.replace("Positivelys |", ""))
    }

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        // Inflate the layout for this fragment
        val inflate = inflater.inflate(R.layout.fragment_web_page, container, false)

        WebView.setWebContentsDebuggingEnabled(true)

        var databasePath = inflate.context.packageManager.getPackageInfo(
            inflate.context.packageName,
            0
        ).applicationInfo.dataDir
        val appRequest = AppRequest(fragmentUrl.toString(), "GET")

        val appService = AppService(databasePath, inflate.context.filesDir.absolutePath)
        val response = appService.makeRequest(appRequest)
        val body = response.body ?: ""

        webView = inflate.findViewById(R.id.fragment_web_view)
        webView.addJavascriptInterface(WebAppInterface(inflate.context as AppPageActivity), "Android")
        val webViewClient = object : WebViewClient() {
            override fun onPageFinished(view: WebView?, url: String?) {
                super.onPageFinished(view, url)
                setTitle()
            }
        }

        webView.webViewClient = webViewClient
        webView.settings.javaScriptEnabled = true
        webView.loadDataWithBaseURL(
            "file:///android_asset/",
            body,
            "text/html",
            "base64",
            "https://positivelys.com"
        )

        return inflate
    }
}
