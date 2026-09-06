import 'package:flutter/material.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: ListView(
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: const [
                Text('ASTRA NEST', style: TextStyle(fontWeight: FontWeight.bold, letterSpacing: 1)),
                Icon(Icons.person_outline),
              ],
            ),
            const SizedBox(height: 16),
            const Text('Good afternoon', style: TextStyle(color: Colors.grey)),
            const SizedBox(height: 24),
            ElevatedButton.icon(
              onPressed: () {
                // TODO: wire to image_picker + ApiService upload
              },
              icon: const Icon(Icons.upload),
              label: const Text('Upload a new report'),
              style: ElevatedButton.styleFrom(
                minimumSize: const Size(double.infinity, 48),
                backgroundColor: Colors.indigo,
              ),
            ),
            const SizedBox(height: 20),
            Row(
              children: const [
                _StatCard(value: '11', label: 'TRACKED'),
                SizedBox(width: 8),
                _StatCard(value: '3', label: 'FLAGGED'),
                SizedBox(width: 8),
                _StatCard(value: '4', label: 'REPORTS'),
              ],
            ),
            const SizedBox(height: 24),
            const Text('LATEST REPORT', style: TextStyle(fontSize: 12, color: Colors.grey)),
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                border: Border.all(color: Colors.grey.shade300),
                borderRadius: BorderRadius.circular(8),
              ),
              child: const Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Hemoglobin: 11.8 g/dL ↓'),
                  SizedBox(height: 8),
                  Text('LDL Cholesterol: 148 mg/dL ↑'),
                  SizedBox(height: 8),
                  Text('Glucose (Fasting): 112 mg/dL ↑'),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _StatCard extends StatelessWidget {
  final String value;
  final String label;
  const _StatCard({required this.value, required this.label});

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 16),
        decoration: BoxDecoration(
          border: Border.all(color: Colors.grey.shade300),
          borderRadius: BorderRadius.circular(8),
        ),
        child: Column(
          children: [
            Text(value, style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
            Text(label, style: const TextStyle(fontSize: 10, color: Colors.grey)),
          ],
        ),
      ),
    );
  }
}
