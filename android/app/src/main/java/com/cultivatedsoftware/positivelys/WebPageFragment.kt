package com.cultivatedsoftware.positivelys

import android.os.Bundle
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.webkit.WebView
import android.webkit.WebViewClient
import com.cultivatedsoftware.positivelys.models.AppRequest
import com.cultivatedsoftware.positivelys.services.AppService

const val WEB_PAGE_FRAGMENT_URL = "url"

class WebPageFragment : Fragment(R.layout.fragment_web_page) {
    var fragmentUrl: String? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        arguments?.let {
            fragmentUrl = it.getString(WEB_PAGE_FRAGMENT_URL)
        }

    }

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?,
                              savedInstanceState: Bundle?): View? {
        // Inflate the layout for this fragment
        val inflate = inflater.inflate(R.layout.fragment_web_page, container, false)


        var databasePath = inflate.context.packageManager.getPackageInfo(inflate.context.packageName, 0).applicationInfo.dataDir
        val appRequest = AppRequest(fragmentUrl.toString(), "GET")

        val appService = AppService(databasePath)
        val response = appService.makeRequest(appRequest)
        val body = response.body ?: ""

        val myWebView: WebView = inflate.findViewById(R.id.fragment_web_view)
        myWebView.addJavascriptInterface(WebAppInterface(inflate.context), "Android")
        myWebView.webViewClient = WebViewClient()
        myWebView.settings.javaScriptEnabled = true
        myWebView.loadDataWithBaseURL(
            "https://positivelys.com",
            body,
            "text/html",
            "base64",
            "https://positivelys.com"
        )
        return inflate
    }
}