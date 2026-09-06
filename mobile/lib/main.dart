import 'package:flutter/material.dart';
import 'screens/home_screen.dart';
import 'screens/trends_screen.dart';
import 'screens/chat_screen.dart';
import 'screens/reports_screen.dart';

void main() {
  runApp(const AstraNestApp());
}

class AstraNestApp extends StatelessWidget {
  const AstraNestApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Astra Nest',
      theme: ThemeData(
        primarySwatch: Colors.indigo,
        scaffoldBackgroundColor: Colors.white,
      ),
      home: const RootTabs(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class RootTabs extends StatefulWidget {
  const RootTabs({super.key});

  @override
  State<RootTabs> createState() => _RootTabsState();
}

class _RootTabsState extends State<RootTabs> {
  int _index = 0;

  final _screens = const [
    HomeScreen(),
    TrendsScreen(),
    ChatScreen(),
    ReportsScreen(),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _screens[_index],
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _index,
        onTap: (i) => setState(() => _index = i),
        type: BottomNavigationBarType.fixed,
        items: const [
          BottomNavigationBarItem(icon: Icon(Icons.home_outlined), label: 'Home'),
          BottomNavigationBarItem(icon: Icon(Icons.show_chart), label: 'Trends'),
          BottomNavigationBarItem(icon: Icon(Icons.chat_bubble_outline), label: 'Chat'),
          BottomNavigationBarItem(icon: Icon(Icons.description_outlined), label: 'Reports'),
        ],
      ),
    );
  }
}
