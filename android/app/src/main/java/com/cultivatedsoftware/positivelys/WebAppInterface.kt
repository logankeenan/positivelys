package com.cultivatedsoftware.positivelys

import android.content.Context
import android.content.Intent
import android.webkit.JavascriptInterface
import com.cultivatedsoftware.positivelys.models.AppRequest
import com.cultivatedsoftware.positivelys.services.AppService
import com.google.gson.Gson


class WebAppInterface(private val mContext: Context) {
    val gson: Gson = Gson()


    @JavascriptInterface
    fun makeAppRequest(appRequestAsJson: String) {
        val databasePath =
            mContext.packageManager.getPackageInfo(mContext.packageName, 0).applicationInfo.dataDir
        val appService = AppService(databasePath)

        val appRequest = gson.fromJson<AppRequest>(appRequestAsJson, AppRequest::class.java)

        var response = appService.makeRequest(appRequest)

        var wasRedirect = false
        if (response.status_code == 302) {
            val location = response.headers?.get("Location")
            val redirectAppRequest = AppRequest(location.toString(), "GET")
            response = appService.makeRequest(redirectAppRequest)

            wasRedirect = true
        }

        val contentLocation = response.headers?.get("Content-Location")
        val intent = Intent(mContext, AppPageActivity::class.java).apply {
            putExtra(APP_PAGE_ACTIVITY_URL, contentLocation)
        }

        if (wasRedirect) {
            intent.putExtra(APP_PAGE_WAS_REDIRECT, true)
        }

        intent.addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)

        mContext.applicationContext.startActivity(intent)
    }
}
