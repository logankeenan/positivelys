package com.cultivatedsoftware.positivelys

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.database.Cursor
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.MediaStore
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.widget.Toolbar
import androidx.core.app.ActivityCompat
import androidx.core.os.bundleOf
import androidx.fragment.app.add
import androidx.fragment.app.commit
import androidx.fragment.app.replace

const val APP_PAGE_ACTIVITY_URL = "com.cultivatedsoftware.positivelys.APP_PAGE_ACTIVITY_URL"
const val APP_PAGE_WAS_REDIRECT = "com.cultivatedsoftware.positivelys.APP_PAGE_WAS_REDIRECT"
const val IMAGE_REQUEST_CODE = 1
private const val STORAGE_PERMISSION_CODE = 101

class AppPageActivity : AppCompatActivity() {
    var imagePickerPath: String = ""
    var imagePickerInputId: String = ""

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_app_page)

        val toolbar =
            findViewById<Toolbar>(R.id.toolbar)
        setSupportActionBar(toolbar)
        supportActionBar!!.setDisplayShowTitleEnabled(false)

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

            supportFragmentManager.addOnBackStackChangedListener {
                (supportFragmentManager.fragments.first() as WebPageFragment).setTitle()
            }
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)

        if (requestCode == STORAGE_PERMISSION_CODE) {
            setImagePickerInWebView()

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

                val fragmentUrl =
                    (supportFragmentManager.fragments.get(getPreviousFragmentIndex()) as WebPageFragment).fragmentUrl

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

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)

        if (requestCode == IMAGE_REQUEST_CODE) {
            if (resultCode == Activity.RESULT_OK) {
                val selectedImage = data?.getData()

                if (selectedImage == null) {
                    setImagePickerInWebView()
                } else {
                    imagePickerPath = getRealPathFromUri(this, selectedImage).toString()

                    if (isReadStoragePermissionGranted()) {
                        setImagePickerInWebView()
                    }
                }
            }
        }
    }

    private fun setImagePickerInWebView() {
        val get =
            supportFragmentManager.fragments.get(supportFragmentManager.fragments.size - 1) as WebPageFragment
        get.webView.evaluateJavascript(
            "window.positivelys.setImagePickerPath('${imagePickerPath}', '$imagePickerInputId')",
            null
        )

        imagePickerInputId = ""
        imagePickerPath = ""


    }

    private fun getRealPathFromUri(context: Context, uri: Uri?): String? {
        var result: String? = null
        val proj = arrayOf(MediaStore.Images.Media.DATA)
        val cursor: Cursor? =
            uri?.let { context.getContentResolver().query(it, proj, null, null, null) }
        if (cursor != null) {
            if (cursor.moveToFirst()) {
                val columIndex: Int = cursor.getColumnIndexOrThrow(proj[0])
                result = cursor.getString(columIndex)
            }
            cursor.close()
        }
        if (result == null) {
            result = "Not found"
        }
        return result
    }

    private fun getPreviousFragmentIndex(): Int {
        var previousFragmentAfterPopBackStack = 0
        val sizeIsOneBecauseTheCurrentItemWillBePopped = 1

        if (supportFragmentManager.fragments.size != sizeIsOneBecauseTheCurrentItemWillBePopped) {
            previousFragmentAfterPopBackStack =
                supportFragmentManager.fragments.lastIndex - 1
        }
        return previousFragmentAfterPopBackStack
    }

    private fun isReadStoragePermissionGranted(): Boolean {
        return if (Build.VERSION.SDK_INT >= 23) {
            if (checkSelfPermission(Manifest.permission.READ_EXTERNAL_STORAGE)
                == PackageManager.PERMISSION_GRANTED
            ) {
                true
            } else {
                ActivityCompat.requestPermissions(
                    this,
                    arrayOf(Manifest.permission.READ_EXTERNAL_STORAGE),
                    STORAGE_PERMISSION_CODE
                )
                false
            }
        } else {
            true
        }
    }
}