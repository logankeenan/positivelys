package com.cultivatedsoftware.positivelys

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        System.loadLibrary("rust_android")
        var databasePath = packageManager.getPackageInfo(packageName, 0).applicationInfo.dataDir
        Log.d("rust", calldatabase("$databasePath/database.sqlite"))
    }

    external fun calldatabase(to: String): String
}