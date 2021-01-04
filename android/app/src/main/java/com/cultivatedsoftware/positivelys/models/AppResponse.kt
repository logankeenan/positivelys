package com.cultivatedsoftware.positivelys.models

class AppResponse(var status_code: Int) {
    var body: String? = null
    var headers: MutableMap<String, String>? = null
}