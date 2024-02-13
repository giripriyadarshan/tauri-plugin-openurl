package com.giripriyadarshan.openurl

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import androidx.activity.result.ActivityResult
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.annotation.ActivityCallback
import android.content.Intent
import android.net.Uri

@InvokeArg
class PingArgs {
  var value: String? = null
}

@TauriPlugin
class OpenUrlPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun openurl(invoke: Invoke) {
        val url = invoke.parseArgs(PingArgs::class.java)
        val browserIntent = Intent(Intent.ACTION_VIEW, Uri.parse(url.value ?: "default value :("))
        
        startActivityForResult(invoke, browserIntent, "openUrlResult")
    }

    @ActivityCallback
    private fun openUrlResult(invoke: Invoke, result: ActivityResult) {
        invoke.resolve()
    }
}
