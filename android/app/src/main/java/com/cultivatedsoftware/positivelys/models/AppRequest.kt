package com.cultivatedsoftware.positivelys.models

class AppRequest(var uri: String, var method: String) {
    var body: String? = null
    var headers: Map<String, String>? = null
}