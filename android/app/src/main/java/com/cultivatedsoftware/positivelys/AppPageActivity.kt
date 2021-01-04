package com.cultivatedsoftware.positivelys

import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import androidx.core.os.bundleOf
import androidx.fragment.app.add
import androidx.fragment.app.commit
import androidx.fragment.app.replace

const val APP_PAGE_ACTIVITY_URL = "com.cultivatedsoftware.positivelys.APP_PAGE_ACTIVITY_URL"
const val APP_PAGE_WAS_REDIRECT = "com.cultivatedsoftware.positivelys.APP_PAGE_WAS_REDIRECT"

class AppPageActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_app_page)

        if (savedInstanceState == null) {
            var url = intent.getStringExtra(APP_PAGE_ACTIVITY_URL)

            if (url == null) {
                url = "https://positivelys.com/positivelys"
            }

            val bundle = bundleOf(WEB_PAGE_FRAGMENT_URL to url)

            supportFragmentManager.commit {
                setReorderingAllowed(true)
                add<WebPageFragment>(R.id.fragment_container_view, null, bundle)
            }
        }
    }

    override fun onNewIntent(intent: Intent?) {
        super.onNewIntent(intent)
        if (intent != null) {
            var url = intent.getStringExtra(APP_PAGE_ACTIVITY_URL)

            val bundle = bundleOf("url" to url)


            val wasRedirect = intent.getBooleanExtra(APP_PAGE_WAS_REDIRECT, false)

            if (wasRedirect) {
                supportFragmentManager.popBackStack()

                val previousFragmentAfterPopBackStack =
                    supportFragmentManager.fragments.lastIndex - 1

                val fragmentUrl =
                    (supportFragmentManager.fragments.get(previousFragmentAfterPopBackStack) as WebPageFragment).fragmentUrl

                if (fragmentUrl == url) {
                    supportFragmentManager.commit {
                        setReorderingAllowed(true)
                        replace<WebPageFragment>(R.id.fragment_container_view, null, bundle)
                    }
                } else {
                    supportFragmentManager.commit {
                        setReorderingAllowed(true)
                        add<WebPageFragment>(R.id.fragment_container_view, null, bundle)
                        addToBackStack(url)
                    }
                }
            } else {
                supportFragmentManager.commit {
                    setReorderingAllowed(true)
                    add<WebPageFragment>(R.id.fragment_container_view, null, bundle)
                    addToBackStack(url)
                }
            }
        }
    }
}