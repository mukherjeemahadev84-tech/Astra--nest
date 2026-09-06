import 'package:flutter/material.dart';

class ChatScreen extends StatefulWidget {
  const ChatScreen({super.key});

  @override
  State<ChatScreen> createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  final _controller = TextEditingController();
  final List<Map<String, String>> _messages = [
    {
      'role': 'system',
      'text': 'This chat explains your reports in simple terms. It does not diagnose or '
          'recommend treatment — always confirm anything important with your doctor.'
    }
  ];

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Column(
        children: [
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.all(16),
              itemCount: _messages.length,
              itemBuilder: (context, i) {
                final m = _messages[i];
                final isSystem = m['role'] == 'system';
                return Align(
                  alignment: isSystem ? Alignment.center : Alignment.centerLeft,
                  child: Container(
                    margin: const EdgeInsets.symmetric(vertical: 4),
                    padding: const EdgeInsets.all(10),
                    decoration: BoxDecoration(
                      color: isSystem ? Colors.grey.shade200 : Colors.indigo.shade50,
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Text(m['text'] ?? '', style: TextStyle(
                      fontStyle: isSystem ? FontStyle.italic : FontStyle.normal,
                      fontSize: isSystem ? 12 : 14,
                    )),
                  ),
                );
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _controller,
                    decoration: const InputDecoration(hintText: 'Ask about a marker...'),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.send),
                  onPressed: () {
                    // TODO: call ApiService.interpretMarker or a dedicated chat endpoint
                  },
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
