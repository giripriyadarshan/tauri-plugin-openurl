package com.giripriyadarshan.openurl

import android.app.Activity
import android.content.Intent
import android.net.Uri
import androidx.activity.result.ActivityResult
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import app.tauri.plugin.JSObject

@InvokeArg
class OpenUrlArgs {
    var value: String? = null
}

@TauriPlugin
class OpenUrlPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun openUrl(invoke: Invoke) {
        val url = invoke.getString("url", "")
        val browserIntent = Intent(Intent.ACTION_VIEW, Uri.parse(url.value ?: "default value :("))

        startActivityForResult(invoke, browserIntent, "openUrlResult")
    }

    @ActivityCallback
    private fun openUrlResult(invoke: Invoke, result: ActivityResult) {
        invoke.resolve()
    }
}
