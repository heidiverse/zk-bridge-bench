import SwiftUI
import bench_lib

struct ContentView: View {
    let bench = Bench()
    
    @State private var sliderValue: Double = 1
    @State private var showToast = false
    
    @State private var issuanceMs: Int64 = 0
    @State private var presentationMs: Int64 = 0
    @State private var verificationMs: Int64 = 0

	var body: some View {
        ZStack {
            VStack(spacing: 20) {
                Text("Number of iterations: \(Int(sliderValue))")
                Slider(value: $sliderValue, in: 1...10, step: 1)
                    .padding()
                
                Button(action: {
                    Task {
                        issuanceMs = bench.benchIssue(n: Int32(sliderValue))
                        showToastMessage()
                    }
                }) {
                    Text("Bench Issuance")
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                }
                Text("\(issuanceMs)ms")
                
                Button(action: {
                    Task {
                        presentationMs = bench.benchPresent(n: Int32(sliderValue))
                        showToastMessage()
                    }
                }) {
                    Text("Bench Presentation")
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                }
                Text("\(presentationMs)ms")
                
                Button(action: {
                    Task {
                        verificationMs = bench.benchVerify(n: Int32(sliderValue))
                        showToastMessage()
                    }
                }) {
                    Text("Bench Verification")
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                }
                Text("\(verificationMs)ms")
            }
            .padding()
            
            if showToast {
                VStack {
                    Spacer()
                    Text("Done")
                        .padding()
                        .background(Color.black.opacity(0.8))
                        .foregroundColor(.white)
                        .cornerRadius(10)
                        .padding(.bottom, 50)
                        .transition(.opacity)
                }
                .animation(.easeInOut, value: showToast)
            }
        }
	}
    
    func showToastMessage() {
        showToast = true
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            showToast = false
        }
    }
}
