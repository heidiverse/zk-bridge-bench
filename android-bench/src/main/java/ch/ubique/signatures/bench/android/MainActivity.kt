package ch.ubique.signatures.bench.android

import Bench
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
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

    var numIterations by remember { mutableFloatStateOf(1f) }
    var issuanceMs by remember { mutableStateOf<Long?>(null) }
    var presentationMs by remember { mutableStateOf<Long?>(null) }
    var verificationMs by remember { mutableStateOf<Long?>(null) }

    Column(
        modifier = Modifier.padding(8.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Slider(
            value = numIterations,
            onValueChange = { numIterations = it },
            valueRange = 1f..10f,
            steps = 8
        )
        Text("Number of iterations: ${numIterations.roundToInt()}")

        Button(
            onClick = {
                issuanceMs = bench.benchIssue(numIterations.roundToInt())
            }
        ) { Text("Bench Issuance") }
        issuanceMs?.let {
            Text("${it}ms")
        }

        Button(
            onClick = {
                presentationMs = bench.benchPresent(numIterations.roundToInt())
            }
        ) { Text("Bench Presentation") }
        presentationMs?.let {
            Text("${it}ms")
        }

        Button(
            onClick = {
                verificationMs = bench.benchVerify(numIterations.roundToInt())
            }
        ) { Text("Bench Verification") }
        verificationMs?.let {
            Text("${it}ms")
        }
    }
}