import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  // Replace with your Railway deployment URL once live.
  static const String baseUrl = 'https://YOUR-RAILWAY-APP.up.railway.app';

  static Future<Map<String, dynamic>> interpretMarker({
    required String reportId,
    required String markerName,
    required String value,
    required String unit,
    int? cycleDay,
  }) async {
    final uri = Uri.parse('$baseUrl/reports/$reportId/interpret').replace(
      queryParameters: {
        'marker_name': markerName,
        'value': value,
        'unit': unit,
        if (cycleDay != null) 'cycle_day': cycleDay.toString(),
      },
    );
    final res = await http.get(uri);
    return jsonDecode(res.body);
  }

  static Future<Map<String, dynamic>> logSymptom({
    required String userId,
    required String symptom,
    String? note,
  }) async {
    final res = await http.post(
      Uri.parse('$baseUrl/tww/log'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'user_id': userId, 'symptom': symptom, 'note': note}),
    );
    return jsonDecode(res.body);
  }
}
