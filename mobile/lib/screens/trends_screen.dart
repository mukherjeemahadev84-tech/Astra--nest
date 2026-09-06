import 'package:flutter/material.dart';

class TrendsScreen extends StatelessWidget {
  const TrendsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    // TODO: wire real marker history to fl_chart LineChart once backend history endpoint exists.
    return const SafeArea(
      child: Center(child: Text('Trends — marker history over time')),
    );
  }
}
