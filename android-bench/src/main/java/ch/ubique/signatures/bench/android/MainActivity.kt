package ch.ubique.signatures.bench.android

import Bench
import android.os.Bundle
import android.widget.ProgressBar
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.asCoroutineDispatcher
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.newFixedThreadPoolContext
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.withContext
import java.util.concurrent.Executors
import kotlin.coroutines.CoroutineContext
import kotlin.math.roundToInt

class MainActivity : ComponentActivity() {
    val zkpContext = Executors.newFixedThreadPool(10).asCoroutineDispatcher()
//    val zkpContext = newFixedThreadPoolContext(100, "ZKP")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MyApplicationTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    BenchView(zkpContext)
                }
            }
        }
    }
}

@OptIn(ExperimentalCoroutinesApi::class)
@Composable
fun BenchView(zkpContext: CoroutineContext) {
    val bench = Bench()

    val scope = rememberCoroutineScope()

    val context = LocalContext.current

    var numIterations by remember { mutableFloatStateOf(1f) }
    var issuanceMs by remember { mutableLongStateOf(0) }
    var presentationMs by remember { mutableLongStateOf(0) }
    var presentationNativeMs by remember { mutableLongStateOf(0) }
    var verificationMs by remember { mutableLongStateOf(0) }
    var verificationNativeMs by remember { mutableLongStateOf(0) }
    var running by remember { mutableStateOf(false) }

    Column(
        modifier = Modifier.padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(8.dp, alignment = Alignment.CenterVertically),
    ) {
        if(running) {
            CircularProgressIndicator(modifier = Modifier.height(20.dp))
        }
        Text("Number of iterations: ${numIterations.roundToInt()}")
        Slider(
            value = numIterations,
            onValueChange = { numIterations = it },
            valueRange = 1f..100f,
            steps = 98
        )

        Button(
            onClick = {
                running = true
                scope.launch(zkpContext) {
                    issuanceMs = bench.benchIssue(numIterations.roundToInt())
                    withContext(Dispatchers.Main) {
                        running = false
                        Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                    }
                }
            }
        ) { Text("Bench Issuance") }
        Text("${issuanceMs}ms")

        Button(
            onClick = {
                running = true
                scope.launch(zkpContext) {
                    presentationMs = bench.benchPresent(numIterations.roundToInt())
                    withContext(Dispatchers.Main) {
                        running = false
                        Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                    }
                }
            }
        ) { Text("Bench Presentation") }
        Text("${presentationMs}ms")

        Button(
            onClick = {
                running = true
                scope.launch(zkpContext) {
                    presentationNativeMs = bench.benchPresentNative(numIterations.roundToInt())
                    withContext(Dispatchers.Main) {
                        running = false
                        Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                    }
                }
            }
        ) { Text("Bench Presentation (Native)") }
        val presentationSizeNative = remember {  bench.nativePresentationSize }
        Text("${presentationNativeMs}ms [${presentationSizeNative.value}]")

        Button(
            onClick = {
                running = true
                scope.launch(zkpContext) {
                    verificationMs = bench.benchVerify(numIterations.roundToInt())
                    withContext(Dispatchers.Main) {
                        running = false
                        Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                    }
                }
            }
        ) { Text("Bench Verification") }

        Text("${verificationMs}ms ")
        Button(
            onClick = {
                running = true
                scope.launch(zkpContext) {
                    verificationNativeMs = bench.benchVerifyNative(numIterations.roundToInt())
                    withContext(Dispatchers.Main) {
                        running = false
                        Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                    }
                }
            }
        ) { Text("Bench Verification Native") }
        Text("${verificationNativeMs}ms")
    }
}