package ch.ubique.signatures.bench.android

import Bench
import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import kotlin.math.roundToInt

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MyApplicationTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    BenchView()
                }
            }
        }
    }
}

@Composable
fun BenchView() {
    val bench = Bench()

    val scope = rememberCoroutineScope()
    val context = LocalContext.current

    var numIterations by remember { mutableFloatStateOf(1f) }
    var issuanceMs by remember { mutableLongStateOf(0) }
    var presentationMs by remember { mutableLongStateOf(0) }
    var verificationMs by remember { mutableLongStateOf(0) }

    Column(
        modifier = Modifier.padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(8.dp, alignment = Alignment.CenterVertically),
    ) {
        Text("Number of iterations: ${numIterations.roundToInt()}")
        Slider(
            value = numIterations,
            onValueChange = { numIterations = it },
            valueRange = 1f..10f,
            steps = 8
        )

        Button(
            onClick = {
                scope.launch {
                    issuanceMs = bench.benchIssue(numIterations.roundToInt())
                    Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                }
            }
        ) { Text("Bench Issuance") }
        Text("${issuanceMs}ms")

        Button(
            onClick = {
                scope.launch {
                    presentationMs = bench.benchPresent(numIterations.roundToInt())
                    Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                }
            }
        ) { Text("Bench Presentation") }
        Text("${presentationMs}ms")

        Button(
            onClick = {
                scope.launch {
                    verificationMs = bench.benchVerify(numIterations.roundToInt())
                    Toast.makeText(context, "Done", Toast.LENGTH_SHORT).show()
                }
            }
        ) { Text("Bench Verification") }
        Text("${verificationMs}ms")
    }
}