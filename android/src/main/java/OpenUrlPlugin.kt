package com.giripriyadarshan.openurl

import android.app.Activity
import android.content.Intent
import android.webkit.WebView
import android.net.Uri
import androidx.activity.result.ActivityResult
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import org.json.JSONException
import java.util.Collections
import java.util.concurrent.ExecutionException


@InvokeArg
internal class OpenUrlArgs {
  lateinit var Url: String
}

@TauriPlugin
class OpenUrlPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun openUrl(invoke: Invoke) {
        val url = invoke.parseArgs(OpenUrlArgs::class.java)
        val browserIntent = Intent(Intent.ACTION_VIEW, Uri.parse(url.Url))

        startActivityForResult(invoke, browserIntent, "openUrlResult")
    }

    @ActivityCallback
    private fun openUrlResult(invoke: Invoke, result: ActivityResult) {
        invoke.resolve()
    }
}
