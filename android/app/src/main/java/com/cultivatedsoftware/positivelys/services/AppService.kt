package com.cultivatedsoftware.positivelys.services

import com.cultivatedsoftware.positivelys.models.AppContext
import com.cultivatedsoftware.positivelys.models.AppRequest
import com.cultivatedsoftware.positivelys.models.AppResponse
import com.google.gson.Gson

class AppService(var databasePath: String) {

    var gson: Gson = Gson();
    fun makeRequest(appRequest: AppRequest): AppResponse {
        val appRequestAsJson = gson.toJson(appRequest)
        val appContext = AppContext(database_path = "$databasePath/database.sqlite")
        val appContextAsJson = gson.toJson(appContext)

        val appResponseAsJson = makeapprequest(appRequestAsJson, appContextAsJson)
        val response = gson.fromJson<AppResponse>(appResponseAsJson, AppResponse::class.java)


        return response
    }

    init {
        System.loadLibrary("rust_android")
    }

    external fun makeapprequest(j_app_request: String, j_app_context: String): String
}