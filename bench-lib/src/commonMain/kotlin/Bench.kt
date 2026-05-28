import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.plus
import uniffi.signatures_bench.Preparation
import uniffi.signatures_bench.prepareCircuit
import kotlin.time.measureTime

class Bench {
    private val preparation: Preparation = uniffi.signatures_bench.prepare()
    private var vc: String? = null
    private var vcNative: String? = null
    private var circuit: ByteArray? = null
    private var vp: String? = null
    private var vpNative: String? = null
    private var _nativePresentationSize = MutableStateFlow(0)
    var nativePresentationSize = _nativePresentationSize.asStateFlow()
    var scope = CoroutineScope(Dispatchers.Default) + Job()

    private fun issue(): String = uniffi.signatures_bench.zkpIssue(
        issuer = preparation.issuer,
        deviceBinding = preparation.deviceBinding
    )
    private fun issueNative(): String = uniffi.signatures_bench.zkpIssueNative(
        issuer = preparation.issuer,
        deviceBinding = preparation.deviceBinding
    )

    private fun present(vc: String): String {
         return uniffi.signatures_bench.zkpPresent(
            vc,
            issuerPk = preparation.issuer.publicKey,
            provingKeys = preparation.provingKeys,
            publicKey = preparation.dbPublicKey,
            message = preparation.message,
            messageSignature = preparation.messageSignature
        )
    }

    private fun presentNative(vc: String, setup: ByteArray? = null): String =
        uniffi.signatures_bench.zkpPresentNative(
            vc,
            issuerPk = preparation.issuer.publicKey,
            provingKeys = preparation.provingKeys,
            publicKey = preparation.dbPublicKey,
            message = preparation.message,
            messageSignature = preparation.messageSignature,
            setup = setup
        )


    fun benchIssue(n: Int): Long {
        return measureTime {
            for (x in 0..<n) {
                vc = issue()
            }
        }.inWholeMilliseconds / n
    }

    fun benchPresent(n: Int): Long {
        scope. run {
            val vc = this@Bench.vc ?: issue()

            return measureTime {
                for (x in 0..<n) {
                    vp = present(vc)
                }
            }.inWholeMilliseconds / n
        }
    }
    fun benchPresentNative(n: Int) : Long {
        val vc = this@Bench.vcNative ?: issueNative()
        val circuit = this@Bench.circuit ?: prepareCircuit()

        return measureTime {
            for (x in 0..<n) {
                vpNative = presentNative(vc, circuit)
            }
            _nativePresentationSize.update { vpNative?.length ?: 0 }
        }.inWholeMilliseconds / n
    }

     fun benchVerify(n: Int): Long {
        val vc = this@Bench.vc ?: issue()
        val vp = this@Bench.vp ?: present(vc)

        return measureTime {
            for (x in 0..<n) {
                uniffi.signatures_bench.zkpVerify(
                    presentation = vp,
                    issuerPk = preparation.issuer.publicKey,
                    verifyingKeys = preparation.verifyingKeys,
                    message = preparation.message
                )
            }
        }.inWholeMilliseconds / n
    }
    fun benchVerifyNative(n: Int): Long {
        val vc = this@Bench.vcNative ?: issueNative()
        val vp = this@Bench.vpNative ?: presentNative(vc)
        val circuit = this@Bench.circuit ?: prepareCircuit()

        return measureTime {
            for (x in 0..<n) {
                uniffi.signatures_bench.zkpVerifyNative(
                    presentation = vp,
                    issuerPk = preparation.issuer.publicKey,
                    verifyingKeys = preparation.verifyingKeys,
                    message = preparation.message,
                    circuit
                )
            }
        }.inWholeMilliseconds / n
    }

    companion object {
        fun getHello(): String = uniffi.signatures_bench.hello()
    }
}